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

WebUI.callTestCase(findTestCase('CT/TC1_Accesstoken'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('CT/TC2_Creatcart'), [:], FailureHandling.STOP_ON_FAILURE)

ShippingAddressResponse = WS.sendRequest(findTestObject('Postman/Project/Carts/Update actions/SetShippingAddress', [('host') : GlobalVariable.host
            , ('project-key') : 'kv-qa-team', ('cart_id') : GlobalVariable.cart_id, ('cart_version') : GlobalVariable.cart_version]))

WS.verifyResponseStatusCode(ShippingAddressResponse, 200)

if (WS.getResponseStatusCode(ShippingAddressResponse) == 200) {
    println('successfully generated status code 200')
} else {
    println('Failed to generatesuccessfull status code' + WS.getResponseStatusCode(ShippingAddressResponse))
}

//Update cartid & cartversion 
cartid = WS.getElementPropertyValue(ShippingAddressResponse, 'id')

GlobalVariable.cart_id = cartid

cartversion = WS.getElementPropertyValue(ShippingAddressResponse, 'version')

GlobalVariable.cart_version = cartversion

println(GlobalVariable.cart_id)

println(GlobalVariable.cart_version)

