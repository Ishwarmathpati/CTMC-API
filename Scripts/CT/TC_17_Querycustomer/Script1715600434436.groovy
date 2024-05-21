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

QuerycustomerResponse = WS.sendRequest(findTestObject('Postman/Project/Customers/Query customers', [('host') : GlobalVariable.host
            , ('project_key') : GlobalVariable.project_key]))

def ResponseText = QuerycustomerResponse.getResponseText()

//Double DecimalValue = WS.getElementPropertyValue(Response, JsonPath.toString())
//println(DecimalValue)
println(JsonOutput.println(ResponseText))

WS.verifyResponseStatusCode(QuerycustomerResponse, 200, FailureHandling.STOP_ON_FAILURE)

TotalCustomers=WS.getElementPropertyValue(QuerycustomerResponse,'total')

println(TotalCustomers)

if(TotalCustomers>0)
	{
		println("Total customers"+"="+TotalCustomers)
	}
	else {
		println("No products")
		
	}
	
	//Customerid of the first product in the response
	Customerid=WS.getElementPropertyValue(QuerycustomerResponse, 'results[0].id')
	
	println("id"+"="+Customerid)



