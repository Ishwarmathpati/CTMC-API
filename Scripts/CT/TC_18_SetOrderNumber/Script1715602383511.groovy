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

WebUI.callTestCase(findTestCase('CT/TC4_Order'), [:], FailureHandling.STOP_ON_FAILURE)

SetOrderNumberResponse = WS.sendRequest(findTestObject('Postman/Project/Orders/Update actions/SetOrderNumber', [('host') : GlobalVariable.host
            , ('project_key') : GlobalVariable.project_key, ('order_id') : GlobalVariable.order_id, ('order_version') : GlobalVariable.order_version]))

def responseText =SetOrderNumberResponse.getResponseText()

//Double DecimalValue = WS.getElementPropertyValue(Response, JsonPath.toString())
//println(DecimalValue)
println(JsonOutput.println(responseText))

WS.verifyResponseStatusCode(SetOrderNumberResponse, 200, FailureHandling.STOP_ON_FAILURE)

OrderNumber_request="003"

Orderid=WS.getElementPropertyValue(SetOrderNumberResponse, 'id')

println("Orderid"+":"+Orderid)

OrdernumberResponse=WS.getElementPropertyValue(SetOrderNumberResponse,'orderNumber')

println("ordernumber"+":"+OrdernumberResponse)

if (OrderNumber_request==OrdernumberResponse)
{
	println("Order Number got updated")
}
else {
	
   println("Order Number didnt get updated")
}



cartid=WS.getElementPropertyValue(SetOrderNumberResponse,'cart.id')
println("cartid"+":"+cartid)


