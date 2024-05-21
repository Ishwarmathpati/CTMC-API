import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import groovy.json.JsonOutput as JsonOutput

WebUI.callTestCase(findTestCase('CT/TC1_Accesstoken'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('CT/TC3_SetShipping Address'), [:], FailureHandling.STOP_ON_FAILURE)

OrderResponse=WS.sendRequest(findTestObject('Postman/Project/Orders/Create Order', [('host') : GlobalVariable.host, ('project-key') : 'kv-qa-team'
            , ('cart_id') : GlobalVariable.cart_id, ('cart_version') : GlobalVariable.cart_version]))

def responseText = OrderResponse.getResponseText()

//Double DecimalValue = WS.getElementPropertyValue(Response, JsonPath.toString())
//println(DecimalValue)
println(JsonOutput.println(responseText))

WS.verifyResponseStatusCode(OrderResponse, 201)

if (WS.getResponseStatusCode(OrderResponse) == 201) {
    println('successfully generated status code 201')
} else {
    println('Failed to generatesuccessfull status code' + WS.getResponseStatusCode(OrderResponse))
}

orderid = WS.getElementPropertyValue(OrderResponse, 'id')

GlobalVariable.order_id = orderid

orderversion = WS.getElementPropertyValue(OrderResponse, 'version')

GlobalVariable.order_version = orderversion

cartid = WS.getElementPropertyValue(OrderResponse, 'id')

GlobalVariable.cart_id=cartid

println(GlobalVariable.order_id)

println(GlobalVariable.order_version)

println(GlobalVariable.cart_id)





if (WS.getElementPropertyValue(OrderResponse, 'type').equalsIgnoreCase('order')) {
    println('order got created')
} else {
    println('order creation unsuccessfull')
}

